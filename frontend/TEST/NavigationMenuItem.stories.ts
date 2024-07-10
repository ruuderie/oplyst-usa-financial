import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuItem from '../components/ui/navigation-menu/NavigationMenuItem.vue';

const meta = {
  title: 'NavigationMenuItem',
  component: NavigationMenuItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuItem>;

export default meta;
type Story = StoryObj<typeof NavigationMenuItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};