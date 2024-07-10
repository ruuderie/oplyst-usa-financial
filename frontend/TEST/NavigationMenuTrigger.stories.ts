import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuTrigger from '../components/ui/navigation-menu/NavigationMenuTrigger.vue';

const meta = {
  title: 'NavigationMenuTrigger',
  component: NavigationMenuTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuTrigger>;

export default meta;
type Story = StoryObj<typeof NavigationMenuTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};