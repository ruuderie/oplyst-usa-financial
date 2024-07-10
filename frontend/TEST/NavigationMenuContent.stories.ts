import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuContent from '../components/ui/navigation-menu/NavigationMenuContent.vue';

const meta = {
  title: 'NavigationMenuContent',
  component: NavigationMenuContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuContent>;

export default meta;
type Story = StoryObj<typeof NavigationMenuContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};