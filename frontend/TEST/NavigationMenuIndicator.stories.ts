import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuIndicator from '../components/ui/navigation-menu/NavigationMenuIndicator.vue';

const meta = {
  title: 'NavigationMenuIndicator',
  component: NavigationMenuIndicator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuIndicator>;

export default meta;
type Story = StoryObj<typeof NavigationMenuIndicator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};