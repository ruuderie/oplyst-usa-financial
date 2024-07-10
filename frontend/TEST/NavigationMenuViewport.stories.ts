import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuViewport from '../components/ui/navigation-menu/NavigationMenuViewport.vue';

const meta = {
  title: 'NavigationMenuViewport',
  component: NavigationMenuViewport,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuViewport>;

export default meta;
type Story = StoryObj<typeof NavigationMenuViewport>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};